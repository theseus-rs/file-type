use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27350170: FileFormat = FileFormat {
    id: 27_350_170,
    puid: "wikidata/27350170",
    name: "ADRG Transmittal Header File",
    extensions: &["thf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
