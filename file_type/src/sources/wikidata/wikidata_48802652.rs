use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48802652: FileFormat = FileFormat {
    id: 48_802_652,
    source_type: SourceType::Wikidata,
    name: "Aldus Freehand Drawing, version 4",
    extensions: &["fh4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
