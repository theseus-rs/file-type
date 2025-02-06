use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110984365: FileFormat = FileFormat {
    id: 110_984_365,
    source_type: SourceType::Wikidata,
    name: "Corel VideoStudio HTML5 Project File",
    extensions: &["vsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
