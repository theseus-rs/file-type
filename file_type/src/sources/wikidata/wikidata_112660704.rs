use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112660704: FileFormat = FileFormat {
    id: 112_660_704,
    source_type: SourceType::Wikidata,
    name: "Portfolio File",
    extensions: &["bfl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
