#include "EJDBQuery.h"

cpp::result<int64_t, EJDBQuery::EJDBQueryError> EJDBQuery::limit()
{
    int64_t out = 0;
    iwrc rc = jql_get_limit(m_q, &out);

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return cpp::fail(EJDBQueryError::FailedToGetLimit);
    }

    return out;
}

EJDBQuery::EJDBQuery( const std::string &collection,const std::string &query)
    : m_q(nullptr), m_query(query), m_collection(collection)
{
}

EJDBQuery::EJDBQueryError EJDBQuery::init()
{
    iwrc rc = jql_create(&m_q, m_collection.c_str(), m_query.c_str());

    if (rc)
    {
        iwlog_ecode_error3(rc);
        jql_destroy(&m_q);
        m_q = nullptr;
        return EJDBQueryError::FailedToInit;
    }

    return EJDBQueryError::None;
}

EJDBQuery::EJDBQueryError EJDBQuery::setPlaceholderJSON(const std::string &placeholder, int index, const std::string &val)
{
    JBL jbl = nullptr;
    iwrc rc = jbl_from_json(&jbl, val.c_str());
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);

        return EJDBQuery::EJDBQueryError::JSONParsingError;
    }

    rc = jql_set_json_jbl(m_q, placeholder.c_str(), index, jbl);
    if (rc)
    {
        iwlog_ecode_error3(rc);
        jbl_destroy(&jbl);

        return EJDBQuery::EJDBQueryError::FailedToSetJSON;
    }
    jbl_destroy(&jbl);

    return EJDBQuery::EJDBQueryError::None;
}

EJDBQuery::EJDBQueryError EJDBQuery::setRegexp(const std::string &placeholder, int index, const std::string &regexp)
{
    iwrc rc = jql_set_regexp(m_q, placeholder.c_str(), index, regexp.c_str());

    if (rc)
    {
        iwlog_ecode_error3(rc);
        return EJDBQuery::EJDBQueryError::FailedToSetRegExp;
    }

    return EJDBQuery::EJDBQueryError::None;
}

EJDBQuery::EJDBQueryError EJDBQuery::setNull(const std::string &placeholder, int index)
{
    iwrc rc = jql_set_null(m_q, placeholder.c_str(), index);
    if (rc)
    {
        return EJDBQuery::EJDBQueryError::FailedToSetPlaceholder;
    }
    return EJDBQuery::EJDBQueryError::None;
}