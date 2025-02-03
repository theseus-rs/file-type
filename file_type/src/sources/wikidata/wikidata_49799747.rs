use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49799747: FileFormat = FileFormat {
    id: 49_799_747,
    source_type: SourceType::Wikidata,
    name: "Adobe Portable Document Catalog Index File, version 3.2",
    extensions: &["pdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
