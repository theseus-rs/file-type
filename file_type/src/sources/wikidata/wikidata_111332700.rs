use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111332700: FileFormat = FileFormat {
    id: 111_332_700,
    source_type: SourceType::Wikidata,
    name: "Roland S-5xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
