#pragma once
{% if build_type == "lib" and library_type == "SHARED" %}
#if defined(_WIN32)
    #ifdef {{ project_name_upper }}_EXPORTS
        #define {{ project_name_upper }}_API __declspec(dllexport)
    #else
        #define {{ project_name_upper }}_API __declspec(dllimport)
    #endif
#else
    #define {{ project_name_upper }}_API
#endif
{%- endif %}
{% if build_type == "lib" and library_type == "SHARED" %}
{{ project_name_upper }}_API {% endif -%} int Add(int a, int b);
{% if enable_rust %}
void HelloRust(void);
{% endif %}