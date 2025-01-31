use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83443959: FileFormat = FileFormat {
    id: 83_443_959,
    puid: "wikidata/83443959",
    name: "Terse Executable",
    extensions: &["efi", "te"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
