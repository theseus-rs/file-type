use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272295: FileFormat = FileFormat {
    id: 111_272_295,
    source_type: SourceType::Wikidata,
    name: "Ensoniq disk image",
    extensions: &["edx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
