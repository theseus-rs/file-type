use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205511: FileFormat = FileFormat {
    id: 28_205_511,
    source_type: SourceType::Wikidata,
    name: "HP 100LX/200LX icon",
    extensions: &["icn", "xbg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
