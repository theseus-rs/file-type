use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34743436: FileFormat = FileFormat {
    id: 34_743_436,
    puid: "wikidata/34743436",
    name: "Softlib",
    extensions: &["slb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
