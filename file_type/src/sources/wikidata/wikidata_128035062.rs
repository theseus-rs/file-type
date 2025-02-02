use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128035062: FileFormat = FileFormat {
    id: 128_035_062,
    source_type: SourceType::Wikidata,
    name: "Protein Data Bank File 3.3",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
