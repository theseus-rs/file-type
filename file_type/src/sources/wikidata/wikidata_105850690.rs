use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850690: FileFormat = FileFormat {
    id: 105_850_690,
    source_type: SourceType::Wikidata,
    name: "Kaitai Struct language (with rem)",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
