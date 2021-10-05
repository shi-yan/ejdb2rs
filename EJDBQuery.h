
#ifndef EJDB_QUERY_H
#define EJDB_QUERY_H

#include <ejdb2/ejdb2.h>
#include <result.hpp>
#include <iostream>
#include <string>

class EJDBPP;


class EJDBQuery
{
friend class EJDBPP;
private:
    JQL m_q;
    std::string m_query;
    std::string m_collection;

public:
    enum class EJDBQueryError
    {
        None = 0,
        Success = 0,
        FailedToGetLimit,
        FailedToInit,
        FailedToSetPlaceholder,
        UnsupportedPlaceholderType,
        JSONParsingError,
        FailedToSetJSON,
        FailedToSetRegExp,
    };

    cpp::result<int64_t, EJDBQueryError> limit();

    EJDBQuery(const std::string &collection,const std::string &query);

    EJDBQueryError init();

    template <typename T>
    EJDBQueryError setPlaceholder(const std::string &placeholder, int index, const T &val)
    {            std::cout << "set" << std::endl;

        if constexpr(std::is_same<T, int64_t>::value)
        {
            iwrc rc = jql_set_i64(m_q, placeholder.c_str(), index, val);
            std::cout << "set2" << std::endl;
            if (rc)
            {
                iwlog_ecode_error3(rc);
                return EJDBQueryError::FailedToSetPlaceholder;
            }
            return EJDBQueryError::None;
        }
        else if constexpr(std::is_same<T, bool>::value){
            iwrc rc = jql_set_bool(m_q, placeholder.c_str(), index, val);
            if (rc)
            {
                iwlog_ecode_error3(rc);
                return EJDBQueryError::FailedToSetPlaceholder;
            }
            return EJDBQueryError::None;
        }
        else if constexpr(std::is_same<T, bool> ::value) {
            iwrc rc = jql_set_str(m_q, placeholder.c_str(), index, val);
            if (rc) {

                iwlog_ecode_error3(rc);
                return EJDBQueryError::FailedToSetPlaceholder;

            }
            return EJDBQueryError::None;
        }
        else
        {
            static_assert("Unsupported Placeholder Type.");
            return EJDBQueryError::UnsupportedPlaceholderType;
        }

        
    }

    EJDBQueryError setPlaceholderJSON(const std::string &placeholder, int index, const std::string &val);

    EJDBQueryError setRegexp(const std::string &placeholder, int index, const std::string &regexp);

    EJDBQueryError setNull(const std::string &placeholder, int index);

};

#endif