use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111191629: FileFormat = FileFormat {
    id: 111_191_629,
    puid: "wikidata/111191629",
    name: "Viewpoint format",
    extensions: &["vet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
