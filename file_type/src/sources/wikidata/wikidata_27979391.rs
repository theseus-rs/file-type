use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979391: FileFormat = FileFormat {
    id: 27_979_391,
    source_type: SourceType::Wikidata,
    name: "ANSI Animation",
    extensions: &["ans", "vt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
