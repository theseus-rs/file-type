use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473691: FileFormat = FileFormat {
    id: 27_473_691,
    source_type: SourceType::Wikidata,
    name: "BIL/BIP/BSQ Header File",
    extensions: &["hdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
