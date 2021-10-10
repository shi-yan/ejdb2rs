#include "EJDB.h"

EJDBPP::EJDBPP() : m_db(nullptr) { return; }

EJDBPP::EJDBError EJDBPP::init()
{
    iwrc rc = ejdb_init();
    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBPP::EJDBError::FailedToInit;
    }
    return EJDBPP::EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::open(const EJDB_OPTS &opts)
{
    iwrc rc = ejdb_open(&opts, &m_db);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBPP::EJDBError::FailedToOpen;
    }
    return EJDBPP::EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::close()
{
    iwrc rc = ejdb_close(&m_db);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBPP::EJDBError::FailedToClose;
    }
    m_db = nullptr;
    return EJDBPP::EJDBError::None;
}

cpp::result<int64_t, EJDBPP::EJDBError> EJDBPP::putNew(const std::string &collection, const std::string &json)
{
    JBL jbl = 0;
    iwrc rc = jbl_from_json(&jbl, json.c_str());
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);
        return cpp::fail(EJDBPP::EJDBError::JSONParsingError);
    }
    int64_t id = 0;
    rc = ejdb_put_new(m_db, collection.c_str(), jbl, &id);
    jbl_destroy(&jbl);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        return cpp::fail(EJDBPP::EJDBError::FailedToPut);
    }

    return id;
}

EJDBPP::EJDBError EJDBPP::put(const std::string &collection, const std::string &json, uint64_t id)
{
    JBL jbl = 0;
    iwrc rc = jbl_from_json(&jbl, json.c_str());
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);
        return EJDBPP::EJDBError::JSONParsingError;
    }

    rc = ejdb_put(m_db, collection.c_str(), jbl, id);
    jbl_destroy(&jbl);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBPP::EJDBError::FailedToPut;
    }

    return EJDBPP::EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::patch(const std::string &collection, const std::string &json, uint64_t id)
{

    iwrc rc = ejdb_patch(m_db, collection.c_str(), json.c_str(), id);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBError::FailedToPatch;
    }

    return EJDBError::None;
}

cpp::result<std::string, EJDBPP::EJDBError> EJDBPP::get(const std::string &collection, uint64_t id)
{
    JBL jbl = 0;
    iwrc rc = ejdb_get(m_db, collection.c_str(), id, &jbl);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);

        return cpp::fail(EJDBError::FailedToGet);
    }
    else if (jbl == 0)
    {
        return cpp::fail(EJDBError::NotFound);
    }

    IWXSTR *xstr = iwxstr_new();
    rc = jbl_as_json(jbl, jbl_xstr_json_printer, xstr, false);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);
        iwxstr_destroy(xstr);

        return cpp::fail(EJDBError::FailedToConvertJSONToString);
    }

    std::string result(iwxstr_ptr(xstr));
    jbl_destroy(&jbl);
    iwxstr_destroy(xstr);
    return result;
}

cpp::result<std::string, EJDBPP::EJDBError> EJDBPP::info()
{
    JBL jbl = 0;

    iwrc rc = ejdb_get_meta(m_db, &jbl);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);

        return cpp::fail(EJDBError::FailedToGetInfo);
    }
    IWXSTR *xstr = iwxstr_new();
    rc = jbl_as_json(jbl, jbl_xstr_json_printer, xstr, false);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);
        iwxstr_destroy(xstr);

        return cpp::fail(EJDBError::FailedToConvertJSONToString);
    }

    std::string result(iwxstr_ptr(xstr));
    jbl_destroy(&jbl);
    iwxstr_destroy(xstr);
    return result;
}

EJDBPP::EJDBError EJDBPP::del(const std::string &collection, int64_t id)
{

    iwrc rc = ejdb_del(m_db, collection.c_str(), id);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBError::FailedToDel;
    }

    return EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::renameCollection(const std::string &oldCollectionName, const std::string &newCollectionName)
{

    iwrc rc = ejdb_rename_collection(m_db, oldCollectionName.c_str(), newCollectionName.c_str());

    if (rc)
    {
        iwlog_ecode_error3(rc);

        return EJDBPP::EJDBError::FailedToRenameCollection;
    }

    return EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::ensureIndex(const std::string &collection, const std::string &path, ejdb_idx_mode_t mode)
{
    iwrc rc = ejdb_ensure_index(m_db, collection.c_str(), path.c_str(), mode);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBPP::EJDBError::FailedToEnsureIndex;
    }
    return EJDBPP::EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::removeIndex(const std::string &collection, const std::string &path, ejdb_idx_mode_t mode)
{

    iwrc rc = ejdb_remove_index(m_db, collection.c_str(), path.c_str(), mode);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBError::FailedToRemoveIndex;
    }
    return EJDBError::None;
}

EJDBPP::EJDBError EJDBPP::removeCollection(const std::string &collection)
{
    iwrc rc = ejdb_remove_collection(m_db, collection.c_str());

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBError::FailedToRemoveCollection;
    }
    return EJDBError::None;
}

cpp::result<uint64_t, EJDBPP::EJDBError> EJDBPP::onlineBackup(const std::string &fileName)
{

    uint64_t ts = 0;
    iwrc rc = ejdb_online_backup(m_db, &ts, fileName.c_str());

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return cpp::fail(EJDBError::FailedToOnlineBackUp);
    }

    return ts;
}

EJDBPP::~EJDBPP()
{
    if (m_db != nullptr)
    {
        ejdb_close(&m_db);
        m_db = nullptr;
    }
}

static iwrc documents_visitor(EJDB_EXEC *ctx, const EJDB_DOC doc, int64_t *step)
{
    // Print document to stderr

    std::function<iwrc(EJDB_EXEC *, const EJDB_DOC, int64_t *)> *functor = (std::function<iwrc(EJDB_EXEC *, const EJDB_DOC, int64_t *)> *)ctx->opaque;

    return (*functor)(ctx, doc, step);
    //return 0;
}

EJDBPP::EJDBError EJDBPP::exec(const EJDBQuery &q, std::function<iwrc(EJDB_EXEC *, const EJDB_DOC, int64_t *)> callback)
{

    EJDB_EXEC ux = {
        .db = m_db,
        .q = q.m_q,
        .visitor = documents_visitor,
        .opaque = (void *)&callback};

    iwrc rc = ejdb_exec(&ux); // Execute query

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBError::FailedToExec;
    }

    return EJDBError::None;
}