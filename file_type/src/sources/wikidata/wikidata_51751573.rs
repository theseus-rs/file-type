use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51751573: FileFormat = FileFormat {
    id: 51_751_573,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
