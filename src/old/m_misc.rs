
pub fn m_is_path_absolute(path: std::string::String) -> bool {
	path.char_indices().next() == Some((0, '/'))
}