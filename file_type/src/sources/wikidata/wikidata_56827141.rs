use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56827141: FileFormat = FileFormat {
    id: 56_827_141,
    puid: "wikidata/56827141",
    name: "SeqBox file",
    extensions: &["sbx"],
    media_types: &["application/x-sbx"],
    internal_signatures: &[],
    related_formats: &[],
};
