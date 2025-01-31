use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83964875: FileFormat = FileFormat {
    id: 83_964_875,
    puid: "wikidata/83964875",
    name: "Microsoft Access Workgroup Information File",
    extensions: &["mdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
