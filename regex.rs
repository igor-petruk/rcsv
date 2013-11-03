use std::libc::*;

struct regex_matching_pair {
	from: c_int,
	to: c_int
}

struct regex_matching_status {
	status: c_int,
	pairs: *regex_matching_pair
}

struct regex_compilation_status {
	compiled_regex: *c_void,
	error_message: *c_char
}

#[link_args = "-lregex_native -L."]
extern {
    fn regex_compile(regex_pattern: *c_char) -> *regex_compilation_status;
    fn regex_match(regex: *c_void, s_value: *c_char, max_matches: c_uint) -> *regex_matching_status;
    fn regex_free(regex: *c_void);
}

pub struct CompiledRegex{
	desc: *c_void
}

pub enum CompilationResult{
	CompilationError(~str),
	CompilationSuccess(~CompiledRegex)
}

#[unsafe_destructor]
impl Drop for CompiledRegex {
    fn drop(&mut self) {
        #[fixed_stack_segment];
        #[inline(never)];
        unsafe{
	        regex_free(self.desc);
	        free(self.desc as *c_void);
		}
    }
}

#[fixed_stack_segment]
#[inline(never)]
pub fn rmatch(regex: &CompiledRegex, s_value: &str, max_matches: uint)->~[(int,int)]{
  	return unsafe{
  		do s_value.with_c_str |c_buffer| {
			let result = regex_match(regex.desc, c_buffer, max_matches as c_uint);
			let pairs: **regex_matching_pair = &((*result).pairs);
			let mut matches: ~[(int, int)] = ~[];
			if ((*result).status==0){
				for i in range(0u, max_matches){
					let pair = *(::std::ptr::offset((*result).pairs, i as int));	
					if (pair.from == -1){
						break;
					} else {
						matches.push((pair.from as int, pair.to as int));
					}
				}
			}
			free((*result).pairs as *c_void);
			free(result as *c_void);
			matches
		}
	}
}

#[fixed_stack_segment]
#[inline(never)]
pub fn compile(regex_pattern: &str)-> ~CompilationResult{

   	return unsafe {
   		do regex_pattern.with_c_str |c_buffer| {
			let ptr = regex_compile(c_buffer);
			let descriptor = (*ptr).compiled_regex;
			let error_message = ::std::c_str::CString::new((*ptr).error_message, true);
			free(ptr as *c_void);
			match error_message.as_str(){
				Some(error) => ~CompilationError(error.to_owned()),
				None => ~CompilationSuccess(~CompiledRegex{
							desc: descriptor
						})
			}
		}
   	}  
}
