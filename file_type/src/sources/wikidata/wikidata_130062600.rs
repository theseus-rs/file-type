use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130062600: FileFormat = FileFormat {
    id: 130_062_600,
    source_type: SourceType::Wikidata,
    name: "Kal source code file",
    extensions: &["kal"],
    media_types: &["application/kal", "text/kal"],
    signatures: &[],
    related_formats: &[],
};
