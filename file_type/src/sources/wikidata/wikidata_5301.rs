use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5301: FileFormat = FileFormat {
    id: 5_301,
    source_type: SourceType::Wikidata,
    name: "TeX",
    extensions: &["tex"],
    media_types: &["application/x-tex", "math/tex", "text/x-tex"],
    signatures: &[],
    related_formats: &[],
};
