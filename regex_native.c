#include <regex.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
	regex_t* compiled_regex;
	char* error_message;
}regex_compilation_status;

typedef struct {
	int status;
	regmatch_t* matched_pairs;
}regex_matching_status;

char* regex_get_regerror (int errcode, regex_t *compiled){
    size_t length = regerror (errcode, compiled, NULL, 0);
    char *buffer = (char*)malloc (length);
    (void) regerror (errcode, compiled, buffer, length);
    return buffer;
}

void regex_free(regex_t* regex){
	regfree(regex);
}

regex_matching_status* regex_match(regex_t* regex, char* s_value, unsigned int max_matches){
	regex_matching_status* result = (regex_matching_status*)malloc(sizeof(regex_matching_status));	
	result->matched_pairs = (regmatch_t*)malloc(sizeof(regmatch_t)*max_matches);
	result->status = regexec(regex, s_value, max_matches, result->matched_pairs, REG_EXTENDED);
	return result;	
}

regex_compilation_status* regex_compile(char* regex_pattern){
	regex_compilation_status* result = (regex_compilation_status*)malloc(sizeof(regex_compilation_status));
	result->compiled_regex = (regex_t*)malloc(sizeof(regex_t));
	
	int err_code = regcomp(result->compiled_regex, regex_pattern, 0);
	
	if (err_code){
		result->error_message = regex_get_regerror(err_code, result->compiled_regex);
		regex_free(result->compiled_regex);
	}
	return result;
}



