use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131294117: FileFormat = FileFormat {
    id: 131_294_117,
    source_type: SourceType::Wikidata,
    name: "Terraform file format",
    extensions: &["hcl", "tf"],
    media_types: &["application/x-terraform", "application/x-tf"],
    internal_signatures: &[],
    related_formats: &[],
};
