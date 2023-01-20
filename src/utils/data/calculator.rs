pub fn calculate_bounds_for_piece(length: u32, piece_length: u32, piece_index: u32) -> (u32, u32) {
    let begin = piece_index * piece_length;
    let mut end = begin + piece_length;

    if end > length {
        end = length;
    }

    (begin, end)
}

pub fn calculate_piece_size(length: u32, piece_length: u32, piece_index: u32) -> u32 {
    let (begin, end) = calculate_bounds_for_piece(length, piece_length, piece_index);

    end - begin
}