use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757684: FileFormat = FileFormat {
    id: 28_757_684,
    puid: "wikidata/28757684",
    name: "Nintendo Game Boy cartridge SAV file",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
