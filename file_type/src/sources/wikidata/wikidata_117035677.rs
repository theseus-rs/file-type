use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117035677: FileFormat = FileFormat {
    id: 117_035_677,
    puid: "wikidata/117035677",
    name: "TurboCAD for Windows Metafile",
    extensions: &["wmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
