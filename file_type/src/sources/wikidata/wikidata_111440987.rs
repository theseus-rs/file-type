use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440987: FileFormat = FileFormat {
    id: 111_440_987,
    source_type: SourceType::Wikidata,
    name: "Visual Basic UserDocument",
    extensions: &["dob"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
