use genetic_algorithms::structures::{gene::GeneT, genotype::Genotype};

use super::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Gene{
    pub id: i64,
    value: f64,
}
impl GeneT for Gene{
    fn value(&self) -> &f64{
        return &self.value;
    }
}
impl PartialEq for Gene {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}


#[test]
fn gene_get_value(){

    //We create 1 gene and get the value of the gene
    let gene_1 = Gene{id:1,value:10.1};
    assert_eq!(*gene_1.value(), 10.1);
}

#[test]
fn genotype_add_gene(){
    //We create 2 genes and push them into the genotype
    let gene_1 = Gene{id:1,value:10.1};
    let gene_2 = Gene{id:2, value:20.2};

    //We create the genotype
    let mut genotype_1 = Genotype::new();
    genotype_1.add_gene(gene_1);
    genotype_1.add_gene(gene_2);

    //We check that the dna of the genotype has 2 elements
    assert_eq!(genotype_1.dna.len(), 2);

    //We check that the first element is the gene_1 and the second one is gene_2
    assert_eq!(&genotype_1.dna[0], &gene_1);
    assert_eq!(&genotype_1.dna[1], &gene_2);
}