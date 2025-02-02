use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_225: FileFormat = FileFormat {
    id: 225,
    source_type: SourceType::Linguist,
    name: "MATLAB",
    extensions: &["m", "matlab"],
    media_types: &["text/x-octave"],
    internal_signatures: &[],
    related_formats: &[],
};
