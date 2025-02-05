use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55594103: FileFormat = FileFormat {
    id: 55_594_103,
    source_type: SourceType::Wikidata,
    name: "CAChe MolStruct CSF",
    extensions: &["csf"],
    media_types: &["chemical/x-cache-csf"],
    signatures: &[],
    related_formats: &[],
};
