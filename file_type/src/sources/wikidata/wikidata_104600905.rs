use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104600905: FileFormat = FileFormat {
    id: 104_600_905,
    puid: "wikidata/104600905",
    name: "KDBX",
    extensions: &["kdbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
