use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263379: FileFormat = FileFormat {
    id: 111_263_379,
    puid: "wikidata/111263379",
    name: "FXpansion DR-008 drumkit",
    extensions: &["dr8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
