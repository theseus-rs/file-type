use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824050: FileFormat = FileFormat {
    id: 27_824_050,
    source_type: SourceType::Wikidata,
    name: "ar, AIX big archive format variant",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
