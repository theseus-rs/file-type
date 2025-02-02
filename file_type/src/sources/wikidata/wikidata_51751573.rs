use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51751573: FileFormat = FileFormat {
    id: 51_751_573,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
