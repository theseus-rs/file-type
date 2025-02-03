use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111332700: FileFormat = FileFormat {
    id: 111_332_700,
    source_type: SourceType::Wikidata,
    name: "Roland S-5xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
