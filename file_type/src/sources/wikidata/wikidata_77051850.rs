use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_77051850: FileFormat = FileFormat {
    id: 77_051_850,
    source_type: SourceType::Wikidata,
    name: "Cal3D Xml Animation File",
    extensions: &["xaf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
