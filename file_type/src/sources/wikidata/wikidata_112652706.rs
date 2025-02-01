use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652706: FileFormat = FileFormat {
    id: 112_652_706,
    puid: "wikidata/112652706",
    name: "Astound Video Project file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
