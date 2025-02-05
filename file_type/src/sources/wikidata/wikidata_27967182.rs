use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967182: FileFormat = FileFormat {
    id: 27_967_182,
    source_type: SourceType::Wikidata,
    name: "Farandole Composer sample",
    extensions: &["fsm", "usm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
