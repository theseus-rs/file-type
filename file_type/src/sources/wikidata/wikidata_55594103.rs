use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55594103: FileFormat = FileFormat {
    id: 55_594_103,
    source_type: SourceType::Wikidata,
    name: "CAChe MolStruct CSF",
    extensions: &["csf"],
    media_types: &["chemical/x-cache-csf"],
    internal_signatures: &[],
    related_formats: &[],
};
