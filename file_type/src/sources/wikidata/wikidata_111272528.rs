use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272528: FileFormat = FileFormat {
    id: 111_272_528,
    puid: "wikidata/111272528",
    name: "Everest embedded bank file",
    extensions: &["emb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
