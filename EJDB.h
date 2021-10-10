#ifndef EJDB_CPP_H
#define EJDB_CPP_H

#include <string>
#include <ejdb2/ejdb2.h>
#include <result.hpp>

#include "EJDBQuery.h"
#include <functional>

class EJDBPP
{

private:
  EJDB m_db;
public:
  EJDBPP();

  enum class EJDBError
  {
    None = 0,
    Success = 0,
    FailedToInit,
    FailedToOpen,
    FailedToClose,
    JSONParsingError,
    FailedToPut,
    FailedToExec,
    FailedToOnlineBackUp,
    FailedToRemoveCollection,
    FailedToRemoveIndex,
    FailedToEnsureIndex,
    FailedToRenameCollection,
    FailedToDel,
    FailedToGetInfo,
    FailedToConvertJSONToString,
    FailedToGet,
    NotFound,
    FailedToPatch
  };

  static EJDBError init();

  EJDBError open(const EJDB_OPTS &opts);

  EJDBPP::EJDBError close();

  cpp::result<int64_t, EJDBError> putNew(const std::string &collection, const std::string &json);

  EJDBPP::EJDBError put(const std::string &collection, const std::string &json, uint64_t id);

  EJDBPP::EJDBError patch(const std::string &collection, const std::string &json, uint64_t id);

  cpp::result<std::string, EJDBPP::EJDBError> get(const std::string &collection, uint64_t id);

  cpp::result<std::string, EJDBError> info();

  EJDBError del(const std::string &collection, int64_t id);

  EJDBError renameCollection(const std::string &oldCollectionName, const std::string &newCollectionName);

  EJDBError ensureIndex(const std::string &collection, const std::string &path, ejdb_idx_mode_t unique);

  EJDBError removeIndex(const std::string &collection, const std::string &path, ejdb_idx_mode_t mode);

  EJDBError removeCollection(const std::string &collection);

  cpp::result<uint64_t, EJDBError> onlineBackup(const std::string &fileName);

  EJDBError exec(const EJDBQuery &q, std::function<iwrc(EJDB_EXEC *, const EJDB_DOC, int64_t *)> callback);

  ~EJDBPP();
};

#endif