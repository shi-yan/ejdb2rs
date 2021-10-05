#include <iostream>
#include "EJDB.h"
#include <iostream>

//int main()
//{
  /*  std::cout << "test" << std::endl;

    EJDBPP::init();

    EJDBPP db;
    EJDB_OPTS opts = {
        .kv = {
            .path = "example.db",
            .oflags = IWKV_RDONLY}};

    db.open(opts);
    db.ensureIndex("test_col", "/val", EJDB_IDX_UNIQUE | EJDB_IDX_I64);

    std::cout << "insert: " << db.put("test_col", "{\"val\":1,\"mm\":\"1\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":3,\"mm\":\"asf3\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":2,\"mm\":\"asf2\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":23,\"mm\":\"asf4\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":13,\"mm\":\"asf6\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":43,\"mm\":\"asf\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":33,\"mm\":\"asf\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":53,\"mm\":\"asf\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":83,\"mm\":\"asf\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":29,\"mm\":\"asf\"}").value() << std::endl;

    std::cout << "insert: " << db.put("test_col", "{\"val\":26,\"mm\":\"asf\"}").value() << std::endl;

    EJDBQuery q("test_col", "/[val > :val]");
    q.init();
    int64_t val = 2;
    q.setPlaceholder("val", 0, val);

    db.exec(q, [](EJDB_EXEC *context, EJDB_DOC doc, int64_t *step) -> iwrc
            {
                std::cout << "visitor " << doc->id << std::endl;
                //doc.raw

                IWXSTR *xstr = iwxstr_new();
                iwrc rc = jbl_as_json(doc->raw, jbl_xstr_json_printer, xstr, false);

                if (rc)
                {
                    iwlog_ecode_error3(rc);
                    iwxstr_destroy(xstr);
                    return rc;
                }

                std::string result(iwxstr_ptr(xstr));
                iwxstr_destroy(xstr);
                std::cout << result << std::endl;
                *step = 1;
                return 0;
            });

    std::cout << db.info().value() << std::endl;
    db.onlineBackup("example2.db");
    db.close();*/


//}

#define CHECK(rc_)          \
  if (rc_) {                 \
    iwlog_ecode_error3(rc_); \
    return 1;                \
  }

static iwrc documents_visitor(EJDB_EXEC *ctx, const EJDB_DOC doc, int64_t *step) {
  // Print document to stderr
  return jbl_as_json(doc->raw, jbl_fstream_json_printer, stderr, JBL_PRINT_PRETTY);
}

int main() {

  EJDB_OPTS opts = {
    .kv = {
      .path = "example.db"
      //,.oflags = IWKV_TRUNC
    }
  };
  EJDB db;     // EJDB2 storage handle
  int64_t id;  // Document id placeholder
  JQL q = 0;   // Query instance
  JBL jbl = 0; // Json document

  iwrc rc = ejdb_init();
  CHECK(rc);

  rc = ejdb_open(&opts, &db);
  CHECK(rc);

  // First record
  rc = jbl_from_json(&jbl, "{\"name\":\"Bianca\", \"age\":4}");
  RCGO(rc, finish);
  rc = ejdb_put_new(db, "parrots", jbl, &id);
  RCGO(rc, finish);
  jbl_destroy(&jbl);

  // Second record
  rc = jbl_from_json(&jbl, "{\"name\":\"Darko\", \"age\":8}");
  RCGO(rc, finish);
  rc = ejdb_put_new(db, "parrots", jbl, &id);
  RCGO(rc, finish);
  jbl_destroy(&jbl);

  // Now execute a query
  rc =  jql_create(&q, "parrots", "/[age > :age]");
  RCGO(rc, finish);

  EJDB_EXEC ux = {
    .db = db,
    .q = q,
    .visitor = documents_visitor
  };

  // Set query placeholder value.
  // Actual query will be /[age > 3]
  rc = jql_set_i64(q, "age", 0, 3);
  RCGO(rc, finish);

  // Now execute the query
  rc = ejdb_exec(&ux);

finish:
  jql_destroy(&q);
  jbl_destroy(&jbl);
  ejdb_close(&db);
  CHECK(rc);
  return 0;
}