use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272276: FileFormat = FileFormat {
    id: 111_272_276,
    source_type: SourceType::Wikidata,
    name: "Ensoniq Mirage disk image file",
    extensions: &["edm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
