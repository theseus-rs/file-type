use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272291: FileFormat = FileFormat {
    id: 111_272_291,
    source_type: SourceType::Wikidata,
    name: "Ensoniq SQ1/SQ2/KS32 disk image",
    extensions: &["edq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
