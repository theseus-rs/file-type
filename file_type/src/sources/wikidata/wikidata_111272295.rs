use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272295: FileFormat = FileFormat {
    id: 111_272_295,
    source_type: SourceType::Wikidata,
    name: "Ensoniq disk image",
    extensions: &["edx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
