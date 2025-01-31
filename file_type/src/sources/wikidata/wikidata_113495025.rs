use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113495025: FileFormat = FileFormat {
    id: 113_495_025,
    puid: "wikidata/113495025",
    name: "Software602 Printer Configuration File",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
