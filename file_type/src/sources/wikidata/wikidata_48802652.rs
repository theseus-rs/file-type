use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48802652: FileFormat = FileFormat {
    id: 48_802_652,
    source_type: SourceType::Wikidata,
    name: "Aldus Freehand Drawing, version 4",
    extensions: &["fh4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
