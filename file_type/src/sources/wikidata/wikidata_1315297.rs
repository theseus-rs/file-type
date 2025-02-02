use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1315297: FileFormat = FileFormat {
    id: 1_315_297,
    source_type: SourceType::Wikidata,
    name: "QuickTime VR",
    extensions: &["qtvr"],
    media_types: &["video/quicktime"],
    internal_signatures: &[],
    related_formats: &[],
};
