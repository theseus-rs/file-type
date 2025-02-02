use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850690: FileFormat = FileFormat {
    id: 105_850_690,
    source_type: SourceType::Wikidata,
    name: "Kaitai Struct language (with rem)",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
