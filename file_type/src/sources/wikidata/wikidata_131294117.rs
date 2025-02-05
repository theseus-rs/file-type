use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131294117: FileFormat = FileFormat {
    id: 131_294_117,
    source_type: SourceType::Wikidata,
    name: "Terraform file format",
    extensions: &["hcl", "tf"],
    media_types: &["application/x-terraform", "application/x-tf"],
    signatures: &[],
    related_formats: &[],
};
