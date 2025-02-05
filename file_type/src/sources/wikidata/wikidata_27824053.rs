use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824053: FileFormat = FileFormat {
    id: 27_824_053,
    source_type: SourceType::Wikidata,
    name: "ar, AIX small archive format variant",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
