use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117224387: FileFormat = FileFormat {
    id: 117_224_387,
    puid: "wikidata/117224387",
    name: "TurboCAD for DOS 3.0 file",
    extensions: &["tcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
