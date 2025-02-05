use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263379: FileFormat = FileFormat {
    id: 111_263_379,
    source_type: SourceType::Wikidata,
    name: "FXpansion DR-008 drumkit",
    extensions: &["dr8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
