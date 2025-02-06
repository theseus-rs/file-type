use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205511: FileFormat = FileFormat {
    id: 28_205_511,
    source_type: SourceType::Wikidata,
    name: "HP 100LX/200LX icon",
    extensions: &["icn", "xbg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
