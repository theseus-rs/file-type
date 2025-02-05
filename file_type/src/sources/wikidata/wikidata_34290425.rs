use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34290425: FileFormat = FileFormat {
    id: 34_290_425,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences output file",
    extensions: &["spo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
